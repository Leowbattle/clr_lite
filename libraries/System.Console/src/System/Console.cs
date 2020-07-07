using System.Runtime.CompilerServices;

namespace System
{
	public static class Console
	{
		[MethodImpl(MethodImplOptions.InternalCall)]
		public static extern void Write(string s);

		public static void WriteLine(string s)
		{
			Write(s);
			Write("\n");
		}
	}
}