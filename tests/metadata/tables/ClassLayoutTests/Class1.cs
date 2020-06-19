using System;
using System.Runtime.InteropServices;

namespace ClassLayoutTests
{
	public class Class1
	{
		[StructLayout(LayoutKind.Explicit, Size = 42)]
		struct Sized
		{
			[FieldOffset(12)]
			int x;
		}

		float[] data = { 1, 2, 3 };
	}
}
