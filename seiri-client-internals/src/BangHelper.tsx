import * as React from "react";
const helper = ({hidden}: {hidden: boolean}) => (
  <div className={hidden ? "bang-help hidden" : "bang-help"}>
    <table>
      <thead>
        <tr>
          <th>Bang</th>
          <th>Description</th>
          <th>Inputs</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td />
          <td>Track Title Search</td>
          <td>
            The empty bang matches all tracks in the database. In addition, a
            bang-less search matches track titles partially.
          </td>
        </tr>
        <tr>
          <td>
            <code>!!</code>
          </td>
          <td>The group bang</td>
          <td>Another bang expression.</td>
        </tr>
        <tr>
          <td>
            <code>!q</code>
          </td>
          <td>Full Text Search</td>
          <td>Matches track title, album title, artist partially.</td>
        </tr>
        <tr>
          <td>
            <code>!Q</code>
          </td>
          <td>Exact Full Text Search</td>
          <td>Matches track title, album title, artist exactly.</td>
        </tr>
        <tr>
          <td>
            <code>!al</code>
          </td>
          <td>Album Title</td>
          <td>Matches the name of the album partially.</td>
        </tr>
        <tr>
          <td>
            <code>!AL</code>
          </td>
          <td>Exact Album Title</td>
          <td>Matches the name of the album exactly.</td>
        </tr>
        <tr>
          <td>
            <code>!ala</code>
          </td>
          <td>Album Artists</td>
          <td>Matches the name of the album artist partially.</td>
        </tr>
        <tr>
          <td>
            <code>!ALA</code>
          </td>
          <td>Exact Album Artists</td>
          <td>Matches the name of the album artist exactly.</td>
        </tr>
        <tr>
          <td>
            <code>!f</code>
          </td>
          <td>Format</td>
          <td>
            <code>flac, mp3, alac, aac, vorbis, opus, wavpack</code> are self
            explanatory. The special tags <code>flac16, flac24</code> allow for
            distinction between FLAC bitrates, and <code>cbr, vbr</code> allow
            for distinction between constant bitrate MP3 and variable bitrate
            MP3.
          </td>
        </tr>
        <tr>
          <td>
            <code>!br[lt|gt]</code>
          </td>
          <td>Bitrate strictly [Less Than | Greater Than]</td>
          <td>Integer</td>
        </tr>
        <tr>
          <td>
            <code>!c(w|h)[lt|gt]</code>
          </td>
          <td>
            Cover art has (width|height) strictly [Less Than | Greater Than]
          </td>
          <td>Integer</td>
        </tr>
        <tr>
          <td>
            <code>!c</code>
          </td>
          <td>Has cover art in tags</td>
          <td>
            <code>true</code> or <code>false</code>
          </td>
        </tr>
        <tr>
          <td>
            <code>!mb</code>
          </td>
          <td>
            Has{" "}
            <a href="http://musicbrainz.org/" rel="nofollow">
              MusicBrainz
            </a>{" "}
            IDs in tags
          </td>
          <td>
            <code>true</code> or <code>false</code>
          </td>
        </tr>
        <tr>
          <td>
            <code>!dup</code>
          </td>
          <td>Is a duplicate of another track (iTunes-like algorithm)</td>
          <td>
            <code>true</code> or <code>false</code>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
);

export default helper;