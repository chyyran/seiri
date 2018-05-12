using System;
using System.Runtime.InteropServices;

namespace libkatatsuki
{
    public class Lib
    {
        [NativeCallable(EntryPoint = "katatsuki_get_track_data", CallingConvention = CallingConvention.Cdecl)]
        public static CTrack GetTrackData(IntPtr filePathPtr) {
            string filePath = Marshal.PtrToStringUTF8(filePathPtr);
            try {
                Track results = new Track(filePath);
                CTrack marshalledResults = new CTrack(results);
                return marshalledResults;
            } catch {
                return new CTrack();
            }       
        }

        [NativeCallable(EntryPoint = "free_corert", CallingConvention = CallingConvention.Cdecl)]
        public static void Free(IntPtr ptr) {
            Marshal.FreeCoTaskMem(ptr);
        }
    }
}