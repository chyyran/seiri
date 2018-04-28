export declare var window: ElectronWindow;
export default interface ElectronWindow extends Window {
    require<T>(moduleName: string): T;
}
  