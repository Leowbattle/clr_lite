using System;
using System.Runtime.InteropServices;

namespace TypeDefTests
{
	public class Class1
	{
		internal interface Nested { }
	}

	class Subclass : Class1 { }

	abstract class Abstract { }

	sealed class Sealed { }

	class NotBeforeFieldInit
	{
		static NotBeforeFieldInit() { }
	}

	[StructLayout(LayoutKind.Auto, CharSet = CharSet.Unicode)]
	struct UnicodeStrings { }

	[StructLayout(LayoutKind.Explicit)]
	struct Explicit { }
}
