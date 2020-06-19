using System;
using System.Runtime.InteropServices;

namespace ClassLayoutTests
{
	public class Class1
	{
		[StructLayout(LayoutKind.Explicit, Size = 42)]
		struct Sized
		{

		}

		float[] data = { 1, 2, 3 };
	}
}
