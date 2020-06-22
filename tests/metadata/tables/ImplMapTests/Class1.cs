using System;
using System.Runtime.InteropServices;

namespace ImplMapTests
{
	public class Class1
	{
		[DllImport("a")]
		extern static void Hello();

		[DllImport("b", CallingConvention = CallingConvention.Cdecl)]
		extern static void Hello2();

		[DllImport("c", CharSet = CharSet.Unicode)]
		extern static void Hello3();
	}
}
