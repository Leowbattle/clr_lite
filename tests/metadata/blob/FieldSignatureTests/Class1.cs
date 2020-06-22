using System;
using System.Collections.Generic;

namespace FieldSignatureTests
{
	public class Class1<T>
	{
		int Int;
		string Str;
		unsafe int* Ptr;
		DateTime ValueType;
		unsafe TimeSpan* ValuePtr;
		Exception Class;
		T TypeGenericParam;
		int[,] Arr;
		List<int> GenericInstantiation;
		object Obj;
		Guid[] Arr2;
	}
}
