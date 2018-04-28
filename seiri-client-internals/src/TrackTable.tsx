import { ChildProcess } from "child_process";
import { orderBy as _, range } from "lodash";
import * as React from "react";
import Draggable, { DraggableData } from "react-draggable";
import {
  AutoSizer,
  Column,
  RowMouseEventHandlerParams,
  SortDirection,
  SortDirectionType,
  SortIndicator,
  Table,
  TableHeaderProps
} from "react-virtualized";
import "react-virtualized/styles.css"; // only needs to be imported once
import ElectronWindow from "./ElectronWindow";
import "./Table.css";
import { Track, TrackFileType } from "./types";

declare var window: ElectronWindow;

interface TrackTableProps {
  tracks: Track[];
  query: string;
}

interface TrackTableState {
  widths: { [tableKey: string]: number };
  sortBy: string;
  sortDirection: SortDirectionType;
  sortedList: Track[];
  selected: { [index: number]: boolean | undefined };
  lastSelected: number | undefined;
}

const TOTAL_WIDTH = 3000;

// tslint:disable:jsx-no-lambda
class TrackTable extends React.Component<TrackTableProps, TrackTableState> {
  constructor(props: TrackTableProps) {
    super(props);
    const sortBy = "album";
    const sortDirection = SortDirection.ASC;
    this.state = {
      // tslint:disable:object-literal-sort-keys
      widths: {
        track: 0.03,
        title: 0.2,
        album: 0.2,
        artist: 0.2,
        albumArtists: 0.2,
        duration: 0.07,
        fileType: 0.1,
        bitrate: 0.07,
        sampleRate: 0.07,
        hasCoverArt: 0.03,
        coverArtHeight: 0.05,
        coverArtWidth: 0.05,
        source: 0.05,
        musicbrainzTrackId: 0.2,
        updated: 0.07,
        filePath: 0.7
      },
      sortBy,
      sortDirection,
      sortedList: this.sortList({ sortBy, sortDirection }),
      selected: [],
      lastSelected: undefined
    };
  }

  public componentWillReceiveProps(newProps: TrackTableProps) {
    const { sortBy, sortDirection } = this.state;
    if (newProps.query !== this.props.query) {
      this.setState({
        sortedList: this.sortList({ sortBy, sortDirection }),
        selected: []
      });
    } else {
      this.setState({ sortedList: this.sortList({ sortBy, sortDirection }) });
    }
  }

  private rowClassName({ index }: { index: number }) {
    if (index < 0) {
      return "table-row table-header";
    }
    let tableRowClass = "table-row";
    if (!!this.state.selected[index]) {
      tableRowClass += " selected";
    }
    if (index % 2 === 0) {
      tableRowClass += " evenRow";
    } else {
      tableRowClass += " oddRow";
    }
    return tableRowClass;
  }

  private rowGetter = ({ index }: { index: number }) =>
    this.getDatum(this.state.sortedList, index);

  private getDatum(list: Track[], index: number) {
    return list[index] || {};
  }

  private sort({
    sortBy,
    sortDirection
  }: {
    sortBy: string;
    sortDirection: SortDirectionType;
  }) {
    const sortedList = this.sortList({ sortBy, sortDirection });

    this.setState({ sortBy, sortDirection, sortedList });
  }

  private sortList({
    sortBy,
    sortDirection
  }: {
    sortBy: string;
    sortDirection: SortDirectionType;
  }) {
    const list = this.props.tracks;
    return _(
      list,
      [sortBy, "album", "tracknumber"],
      sortDirection.toLowerCase()
    );
  }

  private headerResizeHandler(dataKey: string, event: MouseEvent, { deltaX }: DraggableData) {
    this.resizeRow({
      dataKey,
      deltaX
    })
  }

  private headerRenderer = ({
    columnData,
    dataKey,
    disableSort,
    label,
    sortBy,
    sortDirection
  }: TableHeaderProps) => {
    return (
      <React.Fragment key={dataKey}>
        <div className="ReactVirtualized__Table__headerTruncatedText">
          <span className="table-header-label">{label}</span>
        </div>
        {sortBy === dataKey && <SortIndicator sortDirection={sortDirection} />}
        <Draggable
          axis="x"
          defaultClassName="DragHandle"
          defaultClassNameDragging="DragHandleActive"
          // tslint:disable-next-line:jsx-no-bind
          onDrag={this.headerResizeHandler.bind(this, dataKey)}
          position={{ x: 0 } as any}
        >
          <span className="DragHandleIcon">⋮</span>
        </Draggable>
      </React.Fragment>
    );
  };

  private resizeRow = ({
    dataKey,
    deltaX
  }: {
    dataKey: string;
    deltaX: number;
  }) => {
    window.requestAnimationFrame(() => {
      this.setState(prevState => {
        const prevWidths = prevState.widths;
        const percentDelta = deltaX / TOTAL_WIDTH;

        // This is me being lazy :)
        const nextDataKey = dataKey === "name" ? "location" : "description";

        return {
          widths: {
            ...prevWidths,
            [dataKey]: prevWidths[dataKey] + percentDelta,
            [nextDataKey]: prevWidths[nextDataKey] - percentDelta
          }
        };
      });
    });
  };

  private msToTime(ms: number) {
    const minutes = Math.floor(ms / 60000);
    const seconds = ((ms % 60000) / 1000).toFixed(0);
    return minutes + ":" + (Number(seconds) < 10 ? "0" : "") + seconds;
  }

  private fileTypeString(fileType: TrackFileType) {
    switch (fileType) {
      case TrackFileType.FLAC:
        return "FLAC";
      case TrackFileType.FLAC4:
        return "FLAC (4-bit)";
      case TrackFileType.FLAC8:
        return "FLAC (8-bit)";
      case TrackFileType.FLAC16:
        return "FLAC (16-bit)";
      case TrackFileType.FLAC24:
        return "FLAC (24-bit Hi-Res)";
      case TrackFileType.FLAC32:
        return "FLAC (32-bit Integral)";
      case TrackFileType.MP3_CBR:
        return "MP3 (Constant Bitrate)";
      case TrackFileType.MP3_VBR:
        return "MP3 (Variable Bitrate)";
      case TrackFileType.AAC:
        return "AAC (M4A Audio)";
      case TrackFileType.ALAC:
        return "Apple Lossless";
      case TrackFileType.OPUS:
        return "Opus";
      case TrackFileType.VORBIS:
        return "Vorbis";
      case TrackFileType.WAVPACK:
        return "WavPack";
      case TrackFileType.MONKEYS_AUDIO:
        return "Monkey's Audio";
      case TrackFileType.UNKNOWN:
        return "Unknown";
      default:
        return "";
    }
  }

  private handleDoubleClick(event: RowMouseEventHandlerParams) {
    const track: Track = event.rowData as any;
    const path = window.require<any>("path");
    const open = window.require<
      (target: string, options?: any | undefined) => Promise<ChildProcess>
    >("opn");
    open(path.dirname(track.filePath));
  }

  // tslint:disable:no-shadowed-variable
  private handleClick(event: RowMouseEventHandlerParams) {
    // tslint:disable-next-line:no-console
    const mouseEvent = event.event as React.MouseEvent<any>;
    if (!this.state.lastSelected) {
      const newSelection = !!!this.state.selected[event.index];
      const clearState = [];
      clearState[event.index] = newSelection;
      this.setState({ selected: clearState, lastSelected: event.index });
      return;
    }
    if (mouseEvent.shiftKey) {
      // const selectedIndexes = Object.keys(this.state.selected) as any as number[];
      let newSelectionKeys = [];
      const selected = [];
      const lastSelected = this.state.lastSelected;
      if (event.index > lastSelected) {
        newSelectionKeys = range(lastSelected, event.index + 1);
      } else {
        newSelectionKeys = range(event.index, lastSelected + 1);
      }

      for (const key of newSelectionKeys) {
        selected[key] = true;
      }
      this.setState({ selected });
      return;
    }
    if (mouseEvent.ctrlKey) {
      const selected = this.state.selected;
      selected[event.index] = !!!this.state.selected[event.index];
      this.setState({ selected, lastSelected: event.index });
      return;
    }
    const newSelection = !!!this.state.selected[event.index];
    const clearState = [];
    clearState[event.index] = newSelection;
    this.setState({ selected: clearState, lastSelected: event.index });
    return;
  }

  // tslint:disable-next-line:member-ordering
  public render() {
    return (
      <div className="table-container">
        <AutoSizer disableWidth={true}>
          {({ height }) => (
            <Table
              // tslint:disable-next-line:jsx-no-string-ref
              // autoHeight={true}
              // isScrolling={isScrolling}
              // scrollTop={scrollTop}
              className="Table"
              // tslint:disable-next-line:jsx-no-bind
              rowClassName={this.rowClassName.bind(this)}
              headerClassName="table-header"
              width={TOTAL_WIDTH}
              height={height}
              headerHeight={20}
              overscanRowCount={50}
              rowHeight={20}
              rowCount={this.props.tracks.length}
              onRowDoubleClick={this.handleDoubleClick}
              // tslint:disable-next-line:jsx-no-bind
              onRowClick={this.handleClick.bind(this)}
              // tslint:disable-next-line:jsx-no-bind
              sort={this.sort.bind(this)}
              sortBy={this.state.sortBy}
              sortDirection={this.state.sortDirection}
              // tslint:disable-next-line:jsx-no-bind
              rowGetter={this.rowGetter.bind(this)}
            >
              <Column
                headerRenderer={this.headerRenderer}
                label="#"
                dataKey="trackNumber"
                width={this.state.widths.track * TOTAL_WIDTH}
              />
              <Column
                headerRenderer={this.headerRenderer}
                label="Title"
                dataKey="title"
                width={this.state.widths.title * TOTAL_WIDTH}
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.album * TOTAL_WIDTH}
                label="Album"
                dataKey="album"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.artist * TOTAL_WIDTH}
                label="Artist"
                dataKey="artist"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.albumArtists * TOTAL_WIDTH}
                label="Album Artists"
                dataKey="albumArtists"
                cellRenderer={({ cellData }) => (cellData || []).join(";")}
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.duration * TOTAL_WIDTH}
                label="Duration"
                dataKey="duration"
                cellRenderer={({ cellData }) => this.msToTime(cellData)}
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.fileType * TOTAL_WIDTH}
                label="File Type"
                dataKey="fileType"
                cellRenderer={({ cellData }) => this.fileTypeString(cellData)}
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.bitrate * TOTAL_WIDTH}
                label="Bitrate"
                dataKey="bitrate"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.sampleRate * TOTAL_WIDTH}
                label="Sample Rate"
                dataKey="sampleRate"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.hasCoverArt * TOTAL_WIDTH}
                label="Art"
                dataKey="hasFrontCover"
                cellRenderer={({ cellData }) => (cellData ? "Yes" : "No")}
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.coverArtWidth * TOTAL_WIDTH}
                label="Width"
                dataKey="frontCoverWidth"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.coverArtHeight * TOTAL_WIDTH}
                label="Height"
                dataKey="frontCoverHeight"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.source * TOTAL_WIDTH}
                label="Source"
                dataKey="source"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.musicbrainzTrackId * TOTAL_WIDTH}
                label="MusicBrainz ID"
                dataKey="musicbrainzTrackId"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.updated * TOTAL_WIDTH}
                label="Updated"
                dataKey="updated"
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.filePath * TOTAL_WIDTH}
                label="Path"
                dataKey="filePath"
              />
            </Table>
          )}
        </AutoSizer>
      </div>
    );
  }
}

export default TrackTable;
