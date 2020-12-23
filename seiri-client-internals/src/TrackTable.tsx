import React from "react";

import { orderBy as _, range } from "lodash";
import * as Mousetrap from "mousetrap";
import Draggable, { DraggableData } from "react-draggable";
import { Dispatch } from "redux";
import {

  Column,
  RowMouseEventHandlerParams,
  SortDirection,
  SortDirectionType,
  SortIndicator,
  Table,
  TableCellRenderer,
  TableHeaderProps,
  WindowScroller
} from "react-virtualized";
import "react-virtualized/styles.css"; // only needs to be imported once
import { updateSelectedCount, updateTracksTick } from "./actions";
import "./Table.css";
import { Track, TrackFileType } from "./types";

const toLodashDirection = (direction: SortDirectionType) => {
  switch (direction) {
    case "ASC":
      return "asc";
    case "DESC":
      return "desc";
  }
}

interface TrackTableProps {
  tracks: Track[];
  query: string;
  dispatch?: Dispatch<any>;
  hidden: boolean;
}

interface TrackTableState {
  widths: { [tableKey: string]: number };
  sortBy: string;
  sortDirection: SortDirectionType;
  sortedList: Track[];
  selected: boolean[] | undefined;
  lastSelected: number | undefined;
}

const TOTAL_WIDTH = 3000;

// tslint:disable:jsx-no-lambda
class TrackTable extends React.Component<TrackTableProps, TrackTableState> {
  tableRef: React.RefObject<Table>;

  constructor(props: TrackTableProps) {
    super(props);
    const sortBy = "updated";
    const sortDirection = SortDirection.DESC;
    this.tableRef = React.createRef<Table>();
    this.state = {
      // tslint:disable:object-literal-sort-keys
      widths: {
        track: 0.05,
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
      sortedList: this.sortList({ list: this.props.tracks, sortBy, sortDirection }),
      selected: [],
      lastSelected: undefined
    };
    window.addEventListener("keydown", event => {
      if (event.key === "ArrowUp" || event.key === "ArrowDown") {
        let newSelected = this.state.lastSelected
        if (newSelected === undefined) {
          newSelected = 0
        } else {
          if (event.key === "ArrowDown") newSelected++;
          if (event.key === "ArrowUp") newSelected--;

          if (newSelected < 0) newSelected = 0;
          if (newSelected >= this.props.tracks.length) newSelected = this.props.tracks.length - 1;
        }


        if (event.shiftKey) {
          // const selectedIndexes = Object.keys(this.state.selected) as any as number[];
          let newSelectionKeys = [];
          const selected = this.state.selected ?? [];
          const lastSelected = this.state.lastSelected ?? 0;
          
          if (event.key === "ArrowDown") {
            
          }

          if (newSelected > lastSelected) {
            newSelectionKeys = range(lastSelected, newSelected + 1);
          } else {
            newSelectionKeys = range(newSelected, lastSelected + 1);
          }
          for (const key of newSelectionKeys) {
            selected[key] = true;
          }
          this.setState({ selected, lastSelected: lastSelected });
          return;
        } else {
          const clearState = [];
          clearState[newSelected] = true;
          this.setState({ selected: clearState, lastSelected: newSelected });
          this.tableRef?.current?.scrollToPosition(newSelected);
        }
      }

    });
    Mousetrap.bind(['command+r', 'ctrl+r'], () => {
      // tslint:disable-next-line:no-console
      const tracksToRefresh = this.state.sortedList.filter(
        (track, index) => this.state.selected?.[index] === true
      ).map(track => track.filePath);

      window.seiri.refreshTracks(tracksToRefresh)
      // tslint:disable-next-line:no-console
      console.log("REFRESHED!");
      // tslint:disable-next-line:no-console
      console.log(tracksToRefresh);
      this.setState({ selected: [], lastSelected: undefined });
      this.props.dispatch?.(updateTracksTick.action({}));
      return false;
    });
    this.rowClassName = this.rowClassName.bind(this);
    this.handleClick = this.handleClick.bind(this);
    this.handleDoubleClick = this.handleDoubleClick.bind(this);
    this.sort = this.sort.bind(this);
    this.rowGetter = this.rowGetter.bind(this);
    this.albumArtistCellRenderer = this.albumArtistCellRenderer.bind(this);
    this.durationCellRenderer = this.durationCellRenderer.bind(this);
    this.fileTypeCellRenderer = this.fileTypeCellRenderer.bind(this);
    this.hasCoverArtCellRenderer = this.hasCoverArtCellRenderer.bind(this);
  }

  componentWillUpdate(nextProps: TrackTableProps, nextState: TrackTableState) {
    this.props.dispatch?.(updateSelectedCount({ count: nextState.selected?.filter(s => s).length ?? 0 }));
  }

  componentWillReceiveProps(newProps: TrackTableProps) {
    const { sortBy, sortDirection } = this.state;
    if (newProps.query !== this.props.query || newProps.tracks.length !== this.props.tracks.length) {
      this.setState({
        sortedList: this.sortList({ list: newProps.tracks, sortBy, sortDirection }),
        selected: [],
        lastSelected: undefined
      });
    } else {
      this.setState({ sortedList: this.sortList({ list: newProps.tracks, sortBy, sortDirection }) });
    }
  }

  rowClassName({ index }: { index: number }) {
    if (index < 0) {
      return "table-row table-header";
    }
    let tableRowClass = "table-row";
    if (!!this.state.selected?.[index]) {
      tableRowClass += " selected";
    }
    if (index % 2 === 0) {
      tableRowClass += " evenRow";
    } else {
      tableRowClass += " oddRow";
    }
    return tableRowClass;
  }

  rowGetter = ({ index }: { index: number }) =>
    this.getDatum(this.state.sortedList, index);

  getDatum(list: Track[], index: number) {
    return list[index] || {};
  }

  sort({
    sortBy,
    sortDirection
  }: {
    sortBy: string;
    sortDirection: SortDirectionType;
  }) {
    const sortedList = this.sortList({ list: this.props.tracks, sortBy, sortDirection });

    this.setState({ sortBy, sortDirection, sortedList });
  }

  sortList({
    list,
    sortBy,
    sortDirection
  }: {
    list: Track[];
    sortBy: string;
    sortDirection: SortDirectionType;
  }) {
    return _(
      list,
      [sortBy, "album", "tracknumber"],
      [toLodashDirection(sortDirection), "asc", "asc"]
    );
  }

  headerResizeHandler(dataKey: string, event: MouseEvent, { deltaX }: DraggableData) {
    this.resizeRow({
      dataKey,
      deltaX
    })
  }

  headerRenderer = ({
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
          position={{ x: 0, y: 0 }}
        >
          <span className="DragHandleIcon">⋮</span>
        </Draggable>
      </React.Fragment>
    );
  };

  resizeRow = ({
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

  msToTime(ms: number) {
    const minutes = Math.floor(ms / 60000);
    const seconds = ((ms % 60000) / 1000).toFixed(0);
    return minutes + ":" + (Number(seconds) < 10 ? "0" : "") + seconds;
  }

  fileTypeString(fileType: TrackFileType) {
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
      case TrackFileType.MP3CBR:
        return "MP3 (Constant Bitrate)";
      case TrackFileType.MP3VBR:
        return "MP3 (Variable Bitrate)";
      case TrackFileType.AAC:
        return "AAC (M4A Audio)";
      case TrackFileType.ALAC:
        return "Apple Lossless";
      case TrackFileType.ALAC16:
        return "Apple Lossless (16-bit)";
      case TrackFileType.ALAC24:
        return "Apple Lossless (24-bit Hi-Res)";
      case TrackFileType.AIFF:
        return "AIFF (PCM Audio)";
      case TrackFileType.AIFF4:
        return "AIFF (4-bit PCM)"
      case TrackFileType.AIFF8:
        return "AIFF (8-bit PCM)"
      case TrackFileType.AIFF16:
        return "AIFF (16-bit PCM)"
      case TrackFileType.AIFF24:
        return "AIFF (24-bit PCM)"
      case TrackFileType.AIFF32:
        return "AIFF (32-bit PCM)"
      case TrackFileType.Opus:
        return "Opus";
      case TrackFileType.Vorbis:
        return "Vorbis";
      case TrackFileType.MonkeysAudio:
        return "Monkey's Audio";
      case TrackFileType.MonkeysAudio16:
        return "Monkey's Audio (16-bit)";
      case TrackFileType.MonkeysAudio24:
        return "Monkey's Audio (24-bit)";
      case TrackFileType.Unknown:
        return "Unknown";
      default:
        return "";
    }
  }

  handleDoubleClick(event: RowMouseEventHandlerParams) {
    const track: Track = event.rowData;
    window.seiri.openTrackFolder(track);
  }

  // tslint:disable:no-shadowed-variable
  handleClick(event: RowMouseEventHandlerParams) {
    // tslint:disable-next-line:no-console
    const mouseEvent = event.event;
    if (this.state.lastSelected === undefined) {
      const newSelection = !!!this.state.selected?.[event.index];
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
      // tslint:disable-next-line:no-console
      console.log(event.index);
      if (selected) {
        selected[event.index] = !!!this.state.selected?.[event.index];
      }
      this.setState({ selected, lastSelected: event.index });
      return;
    }
    const newSelection = !!!this.state.selected?.[event.index];
    const clearState = [];
    clearState[event.index] = newSelection;
    this.setState({ selected: clearState, lastSelected: event.index });
    return;
  }

  albumArtistCellRenderer: TableCellRenderer = ({ cellData }: { cellData?: string[] }) => (cellData || []).join(";")
  durationCellRenderer: TableCellRenderer = ({ cellData }: { cellData?: number }) => this.msToTime(cellData ?? 0)
  fileTypeCellRenderer: TableCellRenderer = ({ cellData }: { cellData?: TrackFileType }) => this.fileTypeString(cellData ?? TrackFileType.Unknown)
  hasCoverArtCellRenderer: TableCellRenderer = ({ cellData }: { cellData?: boolean }) => (cellData ? "Yes" : "No")
  // tslint:disable-next-line:member-ordering
  render() {
    return (
      <div className={this.props.hidden ? "table-container hidden" : "table-container"}>
        <WindowScroller>
          {({ height, isScrolling, scrollTop }: { height: number, isScrolling: boolean, scrollTop: number }) => (
            <Table
              ref={this.tableRef}
              // tslint:disable-next-line:jsx-no-string-ref
              autoHeight={true}
              isScrolling={isScrolling}
              scrollTop={scrollTop}
              className="Table"
              rowClassName={this.rowClassName}
              headerClassName="table-header"
              width={TOTAL_WIDTH}
              height={height}
              headerHeight={20}
              overscanRowCount={50}
              rowHeight={20}
              rowCount={this.props.tracks.length}
              onRowDoubleClick={this.handleDoubleClick}
              onRowClick={this.handleClick}
              sort={this.sort}
              sortBy={this.state.sortBy}
              sortDirection={this.state.sortDirection}
              rowGetter={this.rowGetter}
            >
              <Column
                headerRenderer={this.headerRenderer}
                label="Track"
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
                cellRenderer={this.albumArtistCellRenderer}
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.duration * TOTAL_WIDTH}
                label="Duration"
                dataKey="duration"
                cellRenderer={this.durationCellRenderer}
              />
              <Column
                headerRenderer={this.headerRenderer}
                width={this.state.widths.fileType * TOTAL_WIDTH}
                label="File Type"
                dataKey="fileType"
                cellRenderer={this.fileTypeCellRenderer}
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
                cellRenderer={this.hasCoverArtCellRenderer}
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
        </WindowScroller>
      </div>
    );
  }
}

export default TrackTable;
