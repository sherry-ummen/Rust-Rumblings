using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace ThreadSpeedTestForRust
{
    class Program
    {
        static void Main(string[] args)
        {
            var tasks = new List<Task>();
            var stopwatch = new Stopwatch();
            List<Thread> threads = new List<Thread>();
            stopwatch.Start();
           // Parallel.ForEach(Enumerable.Range(1, 1000), (index) =>
           //{
            for (int i = 0; i < 1000; i++){
                var thread = Task.Factory.StartNew(() =>
                {
                    int count = 0;
                    //string name = Guid.NewGuid().ToString();
                    foreach (var f in Enumerable.Range(1, 5000000))
                    {
                        count += 1;

                    }
                    //Console.WriteLine("Total count {0}", count);
                });
                tasks.Add(thread);
            }
               
               //thread.Wait();
           //});
            Task.WaitAll(tasks.ToArray());
            stopwatch.Stop();
            Console.WriteLine("Threads are done! in {0} ms", stopwatch.Elapsed.Milliseconds);
            stopwatch.Restart();
            TestPinvoke.multi();
            Console.WriteLine("Threads are done! in {0} ms", stopwatch.Elapsed.Milliseconds);
            Console.ReadKey();
        }
    }

    public static class TestPinvoke
    {
        [DllImport("mylib.dll")]
        public static extern void multi();
    }
}
