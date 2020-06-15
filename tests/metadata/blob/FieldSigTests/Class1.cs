using System;

namespace FieldSigTests
{
	struct MyStruct { }

	public class Class1<T>
	{
		bool Bool;
		string Str;
		unsafe int* Ptr;
		MyStruct Struct;
		Exception Class;
		Class1<int> GenericInstance;
		T t;
		object o;
		float[] arr;
		int[][] arr2;
		int[,] arr3;

		Class1<MyStruct[,,,]>[] finalBoss;
	}
}
