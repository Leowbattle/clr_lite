namespace System
{
	public class ArgumentNullException : ArgumentException
	{
		public ArgumentNullException()
		{
		}

		public ArgumentNullException(string message) : base(message)
		{
		}
	}
}