using System;
using System.Runtime.InteropServices;

namespace FieldTests
{
	public class Class1
	{
		int Private;
		public int Public;
		static int Static;
		readonly int Readonly;
		const int Const = 1;
		[MarshalAs(UnmanagedType.Bool)]
		int Marshalled;
	}
}
