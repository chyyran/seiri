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
            if (args.Length == 0) {
                Console.WriteLine("Specify file as commandline argument.");
                Environment.Exit(1);
            }

            if (!File.Exists(args[0])) {
                Console.WriteLine($"Cannot find file {args[0]}.");
                Environment.Exit(1);
            }

            var track = new Track(args[0]);
            Console.OutputEncoding = Encoding.UTF8;
            Console.WriteLine(JsonConvert.SerializeObject(track));
        }
    }
}
