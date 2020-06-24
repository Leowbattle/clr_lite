namespace System
{
    public class Exception
    {
        public virtual string Message { get; }

        public Exception()
        {

        }

        public Exception(string message)
        {
            Message = message;
        }
    }
}