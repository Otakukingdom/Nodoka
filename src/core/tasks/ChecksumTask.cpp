//
// Created by mistlight on 2017/02/04.
//

#include "ChecksumTask.h"

QThreadPool ChecksumTask::threadPoolInstance;

ChecksumTask::ChecksumTask(AudiobookFileProxy *audiobook) {
    this->audiobookFile = audiobook;
}


void ChecksumTask::run() {
    this->audiobookFile->calcAndWriteCheckSum(this->forced);
}

void ChecksumTask::setForced() {
    this->forced = true;
}
