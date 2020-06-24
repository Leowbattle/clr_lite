namespace System.Runtime.Versioning
{
    [AttributeUsage(AttributeTargets.Assembly, AllowMultiple = false, Inherited = false)]
    public sealed class TargetFrameworkAttribute : Attribute
    {
        private readonly string _frameworkName;
        private string? _frameworkDisplayName;

        public TargetFrameworkAttribute(string frameworkName)
        {
            if (frameworkName == null)
                throw new ArgumentNullException(nameof(frameworkName));
            _frameworkName = frameworkName;
        }

        public string FrameworkName => _frameworkName;

        public string? FrameworkDisplayName
        {
            get => _frameworkDisplayName;
            set => _frameworkDisplayName = value;
        }
    }
}