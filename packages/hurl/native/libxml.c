// This callback will prevent from outputting error messages
// It could not be implemented in Rust, because the function is variadic
void silentErrorFunc(void *ctx, const char * msg, ...)
{
}