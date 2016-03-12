/*
The MIT License (MIT)

Copyright (c) 2016 Artemov Ivan (zoxexivo@gmail.com)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

using System;
using System.Linq;
using System.Management;
using System.Threading;
using System.Threading.Tasks;

namespace CPULoad.App.Services.CPU.WMI
{
    public class WmiCounterCpuService : ICpuProvider
    {
        private static readonly ManagementObjectSearcher WmiObject;

        static WmiCounterCpuService()
        {
            WmiObject = new ManagementObjectSearcher("SELECT PercentProcessorTime from Win32_PerfFormattedData_PerfOS_Processor WHERE Name = '_Total'");
        }

        public Task<int> GetCurrentLoadAsync()
        {
            var cpuLoadData = WmiObject.Get().Cast<ManagementObject>().Select(mo => new
            {
                LoadPercent = Convert.ToInt32(mo["PercentProcessorTime"])
            }).FirstOrDefault();

            new AutoResetEvent(false).WaitOne(1000);

            if (cpuLoadData == null)
                Task.FromResult(0);

            return Task.FromResult(cpuLoadData.LoadPercent);
        }
    }
}
