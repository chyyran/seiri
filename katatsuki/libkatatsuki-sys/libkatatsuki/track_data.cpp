#include "track_data.h"
#include "taglib/tstring.h"
#include "taglib/fileref.h"
#include "taglib/tfile.h"
#include "taglib/tbytevector.h"
#include "TrackData.h"
#include <utility>
#include <optional>
#include <iostream>
using namespace std;

extern "C" const int get_file_type(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return trackData->GetFileType();
}

extern "C" track_data * create_track_data(const char* track_path) {
    auto trackData = new TrackData(track_path);
    return reinterpret_cast<track_data*>(trackData);
}

extern "C" void delete_track_data(track_data* track_data) {
    delete reinterpret_cast<TrackData*>(track_data);
}

extern "C" const char* get_title(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return strdup(trackData->GetTitle().to8Bit(true).c_str());
}

extern "C" const char* get_artist(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return strdup(trackData->GetArtist().to8Bit(true).c_str());
}

extern "C" const char* get_album_artist(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return strdup(trackData->GetAlbumArtists().to8Bit(true).c_str());
}

extern "C" const char* get_album(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return strdup(trackData->GetAlbum().to8Bit(true).c_str());
}

extern "C" const char* get_musicbrainz_track_id(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return strdup(trackData->GetMusicBrainzTrackId().to8Bit(true).c_str());
}

extern "C" const unsigned int get_year(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return trackData->GetYear();
}

extern "C" const unsigned int get_track_number(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return trackData->GetTrackNumber();
}

extern "C" const unsigned int get_disc_number(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return trackData->GetDiscNumber();
}

extern "C" const long long get_duration(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return trackData->GetDuration();
}

extern "C" const int get_bitrate(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return trackData->GetBitrate();
}

extern "C" const int get_sample_rate(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return trackData->GetSampleRate();
}

extern "C" const char* get_album_art_all_bytes(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    auto bytes = trackData->GetAlbumArtBytes();
    if (bytes.has_value()) {
        auto byteVectorCopy = new TagLib::ByteVector(bytes.value());
        return byteVectorCopy->data();
    }
    return nullptr;
}

extern "C" const char* get_album_art_bytes(track_data* track_data, size_t size) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    auto bytes = trackData->GetAlbumArtBytes();
    if (bytes.has_value()) {
        char *copy = new char[size];
        memcpy(copy, bytes.value().data(), size);
        return copy;
    }
    return nullptr;
}

extern "C" const bool has_album_art(track_data* track_data) {
    auto* trackData = reinterpret_cast<TrackData*>(track_data);
    return trackData->HasAlbumArt();
}