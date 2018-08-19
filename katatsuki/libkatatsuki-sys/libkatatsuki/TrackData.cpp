#include "TrackData.h"
#include "StringUtils.h"
#include "track_file_type.h"
#include "taglib/tpicture.h"
#include "taglib/tpicturemap.h"
#include "taglib/flacfile.h"
#include "taglib/aifffile.h"
#include "taglib/apefile.h"
#include "taglib/mpegfile.h"
#include "taglib/mp4file.h"
#include "taglib/oggfile.h"
#include "taglib/vorbisfile.h"
#include "taglib/opusfile.h"
#include "taglib/oggflacfile.h"
#include <array>
#include <optional>
#include <iostream>
#include <utility>
#include "taglib/tlist.h"
using namespace std;

TrackData::TrackData(TagLib::FileRef&& _f) : f(_f) {
    f = _f;
}

const TagLib::String TrackData::GetTitle() {
    return this->f.tag()->title();
}

const TagLib::String TrackData::GetArtist() {
    return this->f.tag()->artist();
}

const unsigned int TrackData::GetYear() {
    return this->f.tag()->year();
}


const std::optional<TagLib::ByteVector> TrackData::GetAlbumArtBytes() {
    auto *flacFile = dynamic_cast<TagLib::FLAC::File *>(this->f.file());
    if (flacFile) {
        const TagLib::List<TagLib::FLAC::Picture *> pictureList = flacFile->pictureList();
        // Prefer FrontCover.
        for (const TagLib::FLAC::Picture* picture : pictureList) {
            if (picture->type() == TagLib::FLAC::Picture::FrontCover) {
                return std::optional<TagLib::ByteVector>{picture->data()};
            }
        }

        // Settle for Other.
        for (const TagLib::FLAC::Picture* picture : pictureList) {
            if (picture->type() == TagLib::FLAC::Picture::Other) {
                return std::optional<TagLib::ByteVector>{picture->data()};
            }
        }
    }

    auto pictureMap = this->f.tag()->pictures();
    // Prefer FrontCover, but settle for other.
    if (pictureMap.contains(TagLib::Picture::Type::FrontCover)) {
        auto picture = pictureMap[TagLib::Picture::Type::FrontCover].front();
        return std::optional<TagLib::ByteVector>{picture.data()};
    } else if (pictureMap.contains(TagLib::Picture::Type::Other)) {
        auto picture = pictureMap[TagLib::Picture::Type::Other].front();
        return std::optional<TagLib::ByteVector>{picture.data()};
    }
    return std::nullopt;
}

const unsigned int TrackData::GetTrackNumber() {
    return this->f.tag()->track();
}

const bool TrackData::HasCoverArt() {
    return this->GetAlbumArtBytes().has_value();
}

const int TrackData::GetBitrate() {
    return this->f.audioProperties()->bitrate();
}

const int TrackData::GetSampleRate() {
    return this->f.audioProperties()->sampleRate();
}

const unsigned int TrackData::GetDiscNumber() {
    if(this->f.tag()->properties()["DISCNUMBER"].size() != 0) {
        TagLib::String discNumber = this->f.tag()->properties()["DISCNUMBER"].front();
        return static_cast<unsigned int>(stoul(discNumber.to8Bit()));
    }
    return 1;
}

const long long TrackData::GetDuration() {
    return this->f.audioProperties()->lengthInMilliseconds();
}


const enum track_file_type TrackData::GetFileType() {

    if (auto mp3File = dynamic_cast<TagLib::MPEG::File *>(this->f.file())) {
        // https://github.com/mono/taglib-sharp/blob/b1155885656c9625c2cc6d928b9329e2a5206048/src/TagLib/Mpeg/AudioHeader.cs#L519
        // Mimics taglib-sharp behaviour, even though VBR files may not necessarily have a xing header.
        if (mp3File->audioProperties()->xingHeader()) {
            return track_file_type::MP3VBR;
        }
        return track_file_type::MP3CBR;
    }
    if (auto mp4File = dynamic_cast<TagLib::MP4::File *>(this->f.file())) {
        switch (mp4File->audioProperties()->codec()) {
            case TagLib::MP4::AudioProperties::Codec::AAC:
                return track_file_type::AAC;
            case TagLib::MP4::AudioProperties::Unknown:
                return track_file_type::Unknown;
            case TagLib::MP4::AudioProperties::Codec::ALAC:
                int bitDepth = mp4File->audioProperties()->bitsPerSample();
                return get_alac_type(bitDepth);
        }
    }
    if (auto flacFile = dynamic_cast<TagLib::FLAC::File *>(this->f.file())) {
        int bitDepth = flacFile->audioProperties()->bitsPerSample();
        return get_flac_type(bitDepth);
    }
    if (auto oggFlacFile = dynamic_cast<TagLib::Ogg::FLAC::File *>(this->f.file())) {
        int bitDepth = oggFlacFile->audioProperties()->bitsPerSample();
        return get_flac_type(bitDepth);
    }
    if (auto aiffFile = dynamic_cast<TagLib::RIFF::AIFF::File *>(this -> f.file())) {
        int bitDepth = aiffFile->audioProperties()->bitsPerSample();
        return get_aiff_type(bitDepth);
    }
    if (auto apeFile = dynamic_cast<TagLib::APE::File *>(this->f.file())) {
        int bitDepth = apeFile->audioProperties()->bitsPerSample();
        return get_monkeys_audio_type(bitDepth);
    }
    if (dynamic_cast<TagLib::Ogg::Vorbis::File *>(this->f.file())) {
        return track_file_type::Vorbis;
    }
    if (dynamic_cast<TagLib::Ogg::Opus::File *>(this->f.file())) {
        return track_file_type::Opus;
    }

    const track_file_type result = track_file_type::Unknown;
    return result;
}

const TagLib::String TrackData::GetAlbum() {
    return this->f.tag()->album();
}

const TagLib::String TrackData::GetAlbumArtists() {
    return join(this->f.tag()->properties()["ALBUMARTIST"], ";");
}

const TagLib::String TrackData::GetMusicBrainzTrackId() {
    return this->f.tag()->properties()["MUSICBRAINZ_TRACKID"].front();
}
