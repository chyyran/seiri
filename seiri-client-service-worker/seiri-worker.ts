import ApolloClient from "apollo-boost";
import gql from "graphql-tag";
import * as diff from "deep-diff";
import {} from ".";

interface GlobalScope extends ServiceWorkerGlobalScope {
  trackCache: TrackCache;
}

interface TrackCache {
  allTracks: Array<Track>;
  latestQueryString: String;
  latestQueryTracks: Array<Track>;
}

interface TracksQuery {
  tracks: Array<Track>;
}

interface TrackMessage {
  query?: String;
  refresh?: Array<String>;
}

declare var self: GlobalScope;

const client = new ApolloClient({
  uri: "http://localhost:9234/graphql"
});

enum TrackFileType {
  FLAC,
  FLAC4,
  FLAC8,
  FLAC16,
  FLAC24,
  FLAC32,
  ALAC,
  MP3_CBR,
  MP3_VBR,
  AAC,
  VORBIS,
  OPUS,
  WAVPACK,
  MONKEYS_AUDIO,
  UNKNOWN
}

type Track = {
  filePath: String;
  title: String;
  artist: String;
  albumArtists: Array<String>;
  album: String;
  year: Number;
  trackNumber: Number;
  musicbrainzTrackId: String;
  hasFrontCover: Boolean;
  frontCoverHeight: Number;
  frontCoverWidth: Number;
  bitrate: Number;
  sampleRate: Number;
  source: String;
  discNumber: Number;
  duration: Number;
  fileType: TrackFileType;
};

const TrackFragment = gql`
  fragment TrackFragment on Track {
    filePath
    title
    artist
    albumArtists
    album
    trackNumber
    musicbrainzTrackId
    hasFrontCover
    frontCoverHeight
    frontCoverWidth
    bitrate
    sampleRate
    source
    discNumber
    duration
    fileType
  }
`;

interface TrackData {
  tracks: Array<Track>;
}

self.trackCache = {
  latestQueryString: "",
  latestQueryTracks: [],
  allTracks: []
};

self.addEventListener("install", event => {
  event.waitUntil(self.skipWaiting()); // Activate worker immediately
});

self.addEventListener("activate", function(event) {
  event.waitUntil(self.clients.claim()); // Become available to all pages
  self.setInterval(async () => {
    await seedCache("");
  }, 1000);
  self.setInterval(async () => {
    await seedCache(self.trackCache.latestQueryString);
  }, 1000);
});

self.addEventListener("message", event => {
  let data = event.data as TrackMessage;
  self.setTimeout(async () => {
    await seedCache(data.query || "");
    self.clients.matchAll().then(clients => {
      clients.forEach(client => {
        client.postMessage(self.trackCache);
      });
    });
  }, 0);
  console.log("Sent requested tracks cache.");
});

const seedCache = async (query: String) => {
  let result = await client.query<TracksQuery>({
    query: gql`
      query TracksQuery($query: String!) {
        tracks(query: $query) {
          ...TrackFragment
        }
      }
      ${TrackFragment}
    `,
    variables: {
      query: query
    }
  });

  let compare =
    query === ""
      ? self.trackCache.allTracks
      : self.trackCache.latestQueryTracks;

  let arrayDiff = diff.diff(compare, result.data.tracks);

  if (arrayDiff) {
    if (!query || query === "") {
      self.trackCache.allTracks = result.data.tracks;
    } else {
      self.trackCache.latestQueryTracks = result.data.tracks;
      self.trackCache.latestQueryString = query;
    }
    console.log("New entries detected for diff.");
    self.clients.matchAll().then(clients => {
      clients.forEach(client => {
        client.postMessage(self.trackCache);
      });
    });
  }
};