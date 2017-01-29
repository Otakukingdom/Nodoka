//
// Created by mistlight on 1/29/2017.
//

#include "PlayerScanTask.h"

Core::PlayerScanTask::PlayerScanTask(Core::ScanPlayer *player) {
    this->player = player;
}

void Core::PlayerScanTask::run() {
    this->player->performScan();
}


