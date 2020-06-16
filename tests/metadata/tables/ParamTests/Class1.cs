using System;
using System.Runtime.InteropServices;

namespace ParamTests
{
	public class Class1
	{
		void A(int a) { }
		void B([In] int b) { }
		void C([Out] int c) { }
		void D(int d = 0) { }
		void E([MarshalAs(UnmanagedType.Bool)] int e) { }
	}
}
