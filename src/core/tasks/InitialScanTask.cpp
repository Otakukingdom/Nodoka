//
// Created by mistlight on 1/29/2017.
//

#include "InitialScanTask.h"

Core::InitialScanTask::InitialScanTask(Core::ScanPlayer *player, std::vector<std::shared_ptr<AudiobookProxy>> list) {
    this->player = player;
    this->audiobookList = list;
}

void Core::InitialScanTask::run() {
    for(auto &currentProxy : this->audiobookList) {
        this->player->addAudiobook(currentProxy);
    }
}
