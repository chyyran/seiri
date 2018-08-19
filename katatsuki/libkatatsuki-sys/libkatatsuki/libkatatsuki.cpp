// libkatatsuki.cpp : Defines the entry point for the application.
//
	
#include "libkatatsuki.h"
#include "track_data.h"
#include "taglib/tag.h"
#include "taglib/taglib.h"
#include "taglib/fileref.h"
#include "taglib/tstring.h"
#include "TrackData.h"
#include "taglib/tpicture.h"
#include "track_data.h"

using namespace std;
TrackData* test(const char* s) {
    auto track = new TagLib::FileName(s);
    auto ref = new TagLib::FileRef(*track);
    return new TrackData(std::move(*ref));
}


int main()
{
    auto c_track = create_track_data("track.flac");

	cout << "Hello CMake." << endl;
	cout << get_title(c_track) << endl;
	cout << get_artist(c_track) << endl;
	cout << get_album_art_bytes(c_track, 32) << endl;
   // auto trackData = test("track.flac");
//    auto bytes = trackData->GetAlbumArtBytes();
//    cout << trackData->GetTitle() << endl;
//    cout << trackData->GetAlbum()<< endl;
//    cout << trackData->GetBitrate() << endl;
//    cout << trackData->GetAlbumArtBytes().has_value() << endl;
	//cin.get();
	return 0;
}

