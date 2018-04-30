// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

using System;
using System.Runtime.InteropServices;

namespace libkatatsuki
{
    public class Lib
    {
        [NativeCallable(EntryPoint = "katatsuki_get_track_data", CallingConvention = CallingConvention.Cdecl)]
        public static CTrack GetTrackData(IntPtr filePathPtr) {
            string filePath = Marshal.PtrToStringUni(filePathPtr);
            try {
                Track results = new Track(filePath);
                CTrack marshalledResults = new CTrack(results);
                return marshalledResults;
            } catch {
                return new CTrack();
            }       
        }
    }
}