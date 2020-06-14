using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Security.Permissions;

namespace TypeRefTests
{
	public class Class1
	{
		int x;
		int y;
		int z;

		static void Doit()
		{
			Console.WriteLine("Hello there");
		}

		public class NestedPublic { }

		class NestedPrivate { }

		// internal == Assembly access level
		internal class NestedAssembly { }

		// protected == Family
		protected class NestedFamily { }

		// private protected = FamAndAsm
		private protected class NestedFamAndAsm { }

		// protected internal = FamOrAsm
		protected internal class NestedFamOrAsm { }
	}

	class Subclass : Class1 { }

	class ExtendsExternalType : Exception { }

	sealed class SealedClass { }

	abstract class AbstractClass { }

	interface Interface { }

	struct Struct { }

	[StructLayout(LayoutKind.Explicit)]
	struct ExplicitLayoutStruct { }

	[SpecialName]
	class SpecialName { }

	[Serializable]
	class SerialisableClass { }

	[StructLayout(LayoutKind.Auto, CharSet = CharSet.Unicode)]
	struct UnicodeStruct { }

	[StructLayout(LayoutKind.Auto, CharSet = CharSet.Auto)]
	struct AutoStruct { }

	class HasStaticConstructor
	{
		static HasStaticConstructor() { }
	}
}
