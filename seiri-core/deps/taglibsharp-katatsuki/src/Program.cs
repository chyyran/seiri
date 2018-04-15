using System;
using System.IO;
using System.Text;
using Newtonsoft.Json;
namespace taglibsharp_katatsuki
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length == 0)
            {
                Console.Error.WriteLine("Specify file as commandline argument.");
                Environment.Exit(1);
            }

            if (!File.Exists(args[0]))
            {
                Console.Error.WriteLine($"Cannot find file {args[0]}.");
                Environment.Exit(1);
            }

            try
            {
                var track = new Track(args[0]);
                Console.OutputEncoding = Encoding.UTF8;
                Console.WriteLine(JsonConvert.SerializeObject(track));
                Environment.Exit(0);
            }
            catch (TagLib.UnsupportedFormatException)
            {
                Console.Error.WriteLine($"File {args[0]} is unsupported.");
                Environment.Exit(0);
            }
        }
    }
}
