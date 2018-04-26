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
  console.log("Service Worker Installed.");
  self.setTimeout(async() => updateAllTracks(), 0);
  self.setInterval(async () => {
    await updateAllTracks();
  }, 30000);
  self.setInterval(async () => {
    await updateQueryTracks();
  }, 5000);
});

self.addEventListener("activate", function(event) {
  event.waitUntil(self.clients.claim()); // Become available to all pages
  console.log("Service Worker Activated.");
});

self.addEventListener("message", event => {
  event.waitUntil(self.clients.claim()); // Become available to all pages
  let data = event.data as TrackMessage;
  self.trackCache.latestQueryString = data.query || "";
  if (self.trackCache.latestQueryString === "") {
    broadcast(getActiveState(self.trackCache.latestQueryString), "state-all")
  } else {
    updateQueryTracks(false)
    .then((send) => {
      if (send) {
        broadcast(getActiveState(self.trackCache.latestQueryString), "state-query")
      }
    })
  }
  console.log("Sent requested tracks cache.");
});

const queryTracks = async (query: String) => {
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
    },
    fetchPolicy: "network-only"
  });
  return result.data.tracks;
};

const getActiveState = (query: String) => {
  if (query === "") {
    return self.trackCache.allTracks;
  }
  return self.trackCache.latestQueryTracks;
};

const broadcast = (payload: any, type: String) => {
  self.clients.matchAll().then(clients => {
    clients.forEach(client => {
      client.postMessage({ type: type, payload: payload });
    });
  });
};

const updateAllTracks = async () => {
  try {
    let tracks = await queryTracks("");
    let arrayDiff = diff.diff(self.trackCache.allTracks, tracks);
    if (arrayDiff) {
      self.trackCache.allTracks = tracks;
      broadcast(arrayDiff, "diff-all");
    }
  } catch (err) {
    broadcast(err.message, "error-all")
  }
};

const updateQueryTracks = async (sendDiff: boolean) : Promise<boolean> => {
  try {
    if (self.trackCache.latestQueryString === "") return;
    let tracks = await queryTracks(self.trackCache.latestQueryString);
    let arrayDiff = diff.diff(self.trackCache.latestQueryTracks, tracks);
    if (arrayDiff) {
      self.trackCache.latestQueryTracks = tracks;
      if (sendDiff) {
        broadcast(arrayDiff, "diff-query");
      }
      return true
    }
  } catch(err) {
    broadcast(err.message, "error-query");
    return false
  }
}
