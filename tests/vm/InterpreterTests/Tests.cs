using System;
using System.Collections.Generic;
using System.Text;

namespace InterpreterTests
{
	class Tests
	{
		void Empty() { }

		int Ldc_I4_M1()
		{
			return -1;
		}

		int Ldc_I4_4()
		{
			return 4;
		}

		int Ldc_I4_S()
		{
			return 100;
		}

		int Ldc_I4()
		{
			return 1000;
		}

		float Ldc_R4()
		{
			return 3.14159f;
		}

		double Ldc_R8()
		{
			return 1.2345678;
		}
	}
}
