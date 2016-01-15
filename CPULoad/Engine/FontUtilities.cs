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

using System.Drawing;

namespace CPULoad.Engine
{
    public static class FontUtilities
    {
        public static Font GetFontForSize(int value)
        {
            if (value >= 100)
            {
                return new Font("Arial", 12);
            }

            return new Font("Tahoma", 17);
        }

        public static Color GetBackgroundColor(int value)
        {
            if (value > 90)
                return Color.FromArgb(210, 255, 13, 0);

            if (value >= 50)
                return Color.FromArgb(210, 255, 72, 00);
            
            return Color.FromArgb(180, 3, 99, 173);
        }
    }
}
