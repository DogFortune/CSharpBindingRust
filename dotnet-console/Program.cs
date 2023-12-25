using CsBindgen;

namespace dotnet_console;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("通常のC#");
        CalcPoint(10000000);
        Console.WriteLine("Rust呼び出し");
        NativeMethods.calcpoint(10000000);
        Console.WriteLine("C呼び出し");
        Libfibo.fibo(20);
    }

    /// <summary>
    ///     C#での計測
    /// </summary>
    static void CalcPoint(int points)
    {
        var src = new byte[points * 3];
        var index = 0;

        for (var i = 0; i < points; i++)
        {
            src[index++] = 1;
            src[index++] = 2;
            src[index++] = 3;
        }

        foreach (var i in Enumerable.Range(0, 9))
        {
            Console.WriteLine(src[i]);
        }
    }
}
