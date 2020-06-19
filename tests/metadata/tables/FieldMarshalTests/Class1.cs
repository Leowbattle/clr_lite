using System;
using System.Runtime.InteropServices;

namespace HasFieldMarshalTests
{
	[StructLayout(LayoutKind.Sequential)]
	struct Marshalled
	{
		int x;

		[MarshalAs(UnmanagedType.LPStr)]
		string s;
	}

	public class Class1
	{
		[DllImport("nice")]
		static extern void NativeMethod([MarshalAs(UnmanagedType.LPArray)] int[] arr);

		[DllImport("nice")]
		static extern void NativeMethod2([MarshalAs(UnmanagedType.LPArray, SizeConst = 5)] int[] arr2);

		[DllImport("nice")]
		static extern void NativeMethod3([MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 1)] int[] arr3, int size);

		[DllImport("nice")]
		static extern void NativeMethod4([MarshalAs(UnmanagedType.LPArray, SizeConst = 10, SizeParamIndex = 1)] int[] arr4, int size);
	}
}
