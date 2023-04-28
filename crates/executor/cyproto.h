#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef enum CyprotoError {
  None,
  BufferOverflow,
  Postcard,
} CyprotoError;

typedef struct DriveDone {
  float total_distance;
  bool bump_detected;
  bool cliff_detected;
} DriveDone;

typedef struct DriveCommand {
  float distance;
  uint16_t speed;
} DriveCommand;

typedef struct TurnCommand {
  float angle;
  uint16_t speed;
} TurnCommand;

typedef struct ScanCommand {
  uint8_t start_angle;
  uint8_t end_angle;
  uint8_t fidelity;
} ScanCommand;

typedef enum CommandRequest_Tag {
  Error,
  Drive,
  Turn,
  Scan,
} CommandRequest_Tag;

typedef struct CommandRequest {
  CommandRequest_Tag tag;
  union {
    struct {
      enum CyprotoError error;
    };
    struct {
      struct DriveCommand drive;
    };
    struct {
      struct TurnCommand turn;
    };
    struct {
      struct ScanCommand scan;
    };
  };
} CommandRequest;

typedef struct ObjectData {
  float distance;
  uint8_t angle;
  float width;
} ObjectData;

typedef struct ScanDone {
  size_t size;
  const struct ObjectData *objects;
} ScanDone;

typedef struct TurnDone {
  float total_angle;
} TurnDone;

enum CyprotoError cyproto_drive_done(struct DriveDone val);

struct CommandRequest cyproto_read_command(void);

enum CyprotoError cyproto_scan_done(struct ScanDone val);

size_t cyproto_scan_size(struct ScanCommand cmd);

enum CyprotoError cyproto_turn_done(struct TurnDone val);
