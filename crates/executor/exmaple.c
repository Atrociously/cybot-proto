#include "cyproto.h"
#include <stdint.h>

DriveDone drive(float distance, uint16_t speed) {
    return (DriveDone) {
        .total_distance = 0,
        .bump_detected = false,
        .cliff_detected = true,
    };
}
TurnDone turn(float angle, uint16_t speed) {
    return (TurnDone) {
        .total_angle = 0,
    };
}
void scan(uint8_t start_angle, uint8_t end_angle, uint8_t fidelity, ScanData data[]) {
    size_t index = 0;
    uint8_t angle = start_angle;

    while (angle <= end_angle) {
        data[index] = (ScanData) {
            .ir_distance = 0,
            .ping_distance = 0,
        };
        index += 1;
        angle += fidelity;
    }
}

int main(void) {
    DriveDone driveRes;
    CommandRequest cmd = cyproto_read_command();

    switch (cmd.tag) {
        case Drive:
            driveRes = drive(cmd.drive.distance, cmd.drive.speed);
            cyproto_drive_done(driveRes);
            break;
        case Turn:
            turn(cmd.turn.angle, cmd.turn.speed);
            break;
        case Scan: {
            size_t size = cyproto_scan_size(cmd.scan);
            ScanData* data = malloc(size * sizeof(ScanData));
            scan(cmd.scan.start_angle, cmd.scan.end_angle, cmd.scan.fidelity, data);
            cyproto_scan_done((ScanDone) { .size = size, .data = data });
            break;
        }
        default:
            break;
    }
}
