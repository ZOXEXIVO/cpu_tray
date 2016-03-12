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
using System.Collections.Concurrent;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Runtime.InteropServices;

namespace CPULoad.Utilities
{
    public class IconGenerator : IDisposable
    {
        private readonly ConcurrentDictionary<int, Icon> _iconCache = new ConcurrentDictionary<int, Icon>();
        
        public Icon GetIcon(int value)
        {
            return _iconCache.GetOrAdd(value, GenerateIcon);
        }

        private Icon GenerateIcon(int value)
        {
            using (var bitmap = new Bitmap(32, 32))
            {
                var graphics = Graphics.FromImage(bitmap);

                graphics.Clear(FontUtilities.GetBackgroundColor(value));

                graphics.SmoothingMode = SmoothingMode.HighQuality;
                graphics.InterpolationMode = InterpolationMode.HighQualityBicubic;
                graphics.PixelOffsetMode = PixelOffsetMode.HighQuality;

                var sf = new StringFormat { LineAlignment = StringAlignment.Center, Alignment = StringAlignment.Center };

                graphics.DrawString(Convert.ToString(value), FontUtilities.GetFontForSize(value), Brushes.White, new Rectangle(0, 0, 35, 35), sf);

                var hIcon = bitmap.GetHicon();

                return Icon.FromHandle(hIcon);
            }
        }
        
        [DllImport("user32.dll", CharSet = CharSet.Auto)]
        extern static bool DestroyIcon(IntPtr handle);

        public void Dispose()
        {
            foreach (var icon in _iconCache)
            {
                try
                {
                    if (icon.Value != null)
                        DestroyIcon(icon.Value.Handle);
                }
                catch
                {
                    // ignored
                }
            }
        }
    }
}
