// @ts-ignore
import Markdown from "react-markdown";
import './BangHelper.css';

const helper = ({ hidden }: { hidden: boolean }) => (
  <div className={hidden ? "bang-help hidden" : "bang-help"}>
    <div>
      <Markdown>{`
# *seiri* Bang Reference
Bangs can be used to query your library in more specific ways. Bangs start with an exclamation mark, and
take a parameter enclosed in curly braces, for example

**!t{Hotel California}**.

Searching without a bang is equivalent to the *Full Text Search* bang (**!q**). 
## Search Bangs
The following bangs accept a search term case insensitively, and can be capitalized for case-sensitive exact matches.

**!q / !Q** *Full Text Search* (Title, Album, Artists, Album Artists)

**!t / !T** *Title Search* 

**!al / !AL** *Album Search*

**!alar / !ALAR** *Album Artists Search*

**!ar / !AR** *Artist Name Search*

**!s** *Source*

## Format Bang
The format bang (**!f**) accepts searching for the following formats.

**flac / flac4 / flac8 / flac16/ flac24 / flac32** *FLAC (4/8/16/24/32-bit)* 

**alac / alac16 / alac24** *ALAC (16/24-bit)* 

**mp3 / cbr / vbr** *MP3 (Constant/Variable bitrate)*

**aac** *M4A Audio (iTunes Plus)*

**vorbis / opus** *Ogg Vorbis / Opus* 

**aiff / aiff4 / aiff8 / aiff16 / aiff24 / aiff32** *AIFF (4/8/16/24/32-bit)* 

**ape / ape8 / ape16 / ape24** *Monkey's Audio (8/16/24-bit)*

## Boolean Bangs
The following bangs accept either a **true** or **false** value. You can also append a backtick (*\`*) as
shorthand for **true**, for example **!dup\`** translates to **!dup{true}**.

**!dup** *Duplicate tracks* 

**!mb** *Tracks have MusicBrainz ID tag* 

**!c** *Tracks have cover art tag* 

## Numerical Tags
These tags take a number, and are used to look up things that are greater than (**gt**) or less than (**lt**)
a value.

**!brlt / !brgt** *Bitrate* 

**!cwlt / !cwgt** *Cover Art Width (pixels)* 

**!chlt / !chgt** *Cover Art Height (pixels)* 

## Duration Tags
These tags take in a duration in the form **0h0m0s**, where **0** is a placeholder for any number. 

**!dlt / !dgt** *Track duration* 

## Updated 

These tags take in a date in the form **YYYY-MM-DD**. 

**!ubf / !uaf** *Updated (before / after)*

## Advanced Usage
Bangs can also be combined using the grouping bang, and logical operators.

For example, **!!{!t{Hotel California} & !ar{The Eagles}} | !!{!t{Hey Jude} & !ar{The Beatles}}** will look for
tracks with the title "Hotel California" and the artist "The Eagles", or tracks with the title "Hey Jude" and 
the artist "The Beatles".
    `}
      </Markdown>
    </div>
  </div>
);

export default helper;