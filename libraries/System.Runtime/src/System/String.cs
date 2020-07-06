using System.Runtime.CompilerServices;

namespace System
{
	public class String
	{
		public static readonly string Empty = "";

		public extern char this[int index]
		{
			[MethodImpl(MethodImplOptions.InternalCall)]
			get;
		}

		public extern int Length
		{
			[MethodImpl(MethodImplOptions.InternalCall)]
			get;
		}
	}
}
