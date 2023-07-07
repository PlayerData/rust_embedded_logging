#include <zephyr/logging/log.h>

static void invoke_logger(uint8_t level, ...)
{
  va_list ap;
  va_start(ap, level);

  log2_generic(level, "%s", ap);

  va_end(ap);
}

void embedded_logging_log(uint8_t level, const char *msg)
{
  invoke_logger(level, msg);
};
