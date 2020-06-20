using System;

namespace MethodImplTests
{
	interface ABC
	{
		void Doit();
	}

	interface DEF
	{
		void Doit();
	}

	public class Class1 : ABC, DEF
	{
		void ABC.Doit()
		{
			throw new NotImplementedException();
		}

		void DEF.Doit()
		{
			throw new NotImplementedException();
		}
	}
}
