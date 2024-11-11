#include <zephyr/logging/log.h>

static void invoke_logger(uint8_t level, ...)
{
  va_list ap;
  va_start(ap, level);

  log_generic(level, "%s", ap);

  va_end(ap);
}

void embedded_logging_log(uint8_t level, const char *msg)
{
  invoke_logger(level, msg);
};

void rust_init_logger(void);

static int embedded_logging_init(void)
{
  rust_init_logger();

  return 0;
}

SYS_INIT(embedded_logging_init, APPLICATION, CONFIG_APPLICATION_INIT_PRIORITY);
