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
using System.Threading.Tasks;
using System.Windows.Forms;
using CPULoad.Engine;

namespace CPULoad.App
{
    public class App
    {
        private static Task _workerTask;

        private readonly NotifyIcon _trayIcon = new NotifyIcon { Text = "CPULoad by Artemov Ivan", Visible = true};

        private static readonly IconGenerator IconGenerator = new IconGenerator();

        public void Start()
        {
            InitContextMenu();

            _workerTask = Task.Factory.StartNew(Task_Worker);
        }

        private void Task_Worker()
        {
            if (_trayIcon == null)
                return;

            while (true)
            {
                var processorValue = ProcessorUtilities.GetCpuUsage();

                var newGeneratedIcon = IconGenerator.GetIcon(processorValue);

                _trayIcon.Icon = newGeneratedIcon;
                _trayIcon.Text = "Processor Load: " + processorValue + "%";
            }
        }

        private void InitContextMenu()
        {
            var contextMenu = new ContextMenu();

            var menuAbout = new MenuItem { Index = 0, Text = "About" };
            menuAbout.Click += menuAbout_Click;

            var menuExit = new MenuItem { Index = 1, Text = "E&xit" };
            menuExit.Click += menuExit_Click;

            contextMenu.MenuItems.AddRange(new[] { menuAbout, menuExit });

            _trayIcon.ContextMenu = contextMenu;
        }


        private static void menuAbout_Click(object sender, EventArgs e)
        {
            MessageBox.Show("Created by Artemov Ivan\r\n\r\nzoxexivo@gmail.com", "CPULoad 2.0");
        }

        private static void menuExit_Click(object Sender, EventArgs e)
        {
            IconGenerator.Dispose();

            try
            {
                _workerTask.Dispose();
            }
            catch
            {
                // ignored
            }

            Application.Exit();
        }
    }
}
