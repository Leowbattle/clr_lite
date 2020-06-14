using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace FieldTests
{
	public class Class1
	{
		int a;
		private int b;
		private protected int c;
		internal int d;
		protected int e;
		protected internal int f;
		public int g;
	}

	class Class2 { }

	enum Fruit
	{
		Apple,
		Banana,
		Orange,
		Grape
	}

	class Class3
	{
		static int MyStatic;
		readonly int MyReadonly;
		const int MyConst = 1;

		[NonSerialized]
		int NotSerialised;

		[SpecialName]
		int SpecialInt;

		int HasDefault = 1;

		[MarshalAs(UnmanagedType.Bool)]
		int HasMarshalInfo;

		// FieldList will be fields.rows().len() + 1
		class Hi { }
	}
}
