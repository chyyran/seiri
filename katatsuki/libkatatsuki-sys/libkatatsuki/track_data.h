#include <stdbool.h>

#pragma once

struct track_data;
typedef struct track_data track_data;
#ifdef __cplusplus
extern "C" {
#endif

track_data *create_track_data(const char *track_path);

void delete_track_data(track_data *track_path);

const char *get_title(track_data *track_path);

const char *get_artist(track_data *track_data);

const char *get_album_artist(track_data *track_data);

const char *get_album(track_data *track_data);

const char *get_musicbrainz_track_id(track_data *track_data);

const unsigned int get_year(track_data *track_data);

const unsigned int get_track_number(track_data *track_data);

const unsigned int get_disc_number(track_data *track_data);

const long long get_duration(track_data *track_data);

const int get_bitrate(track_data *track_data);

const int get_sample_rate(track_data *track_data);

const char *get_album_art_all_bytes(track_data *track_data);

const char *get_album_art_bytes(track_data* track_data, size_t size);

const int get_file_type(track_data *track_data);

const bool has_album_art(track_data *track_data);
#ifdef __cplusplus
}
#endif