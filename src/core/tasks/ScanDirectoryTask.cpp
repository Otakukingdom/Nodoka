//
// Created by mistlight on 1/29/2017.
//

#include <src/core/AudiobookScan.h>
#include "ScanDirectoryTask.h"

void Core::ScanDirectoryTask::run() {
    scanDirectory(this->record, this->audiobook);
}

Core::ScanDirectoryTask::ScanDirectoryTask(QSqlRecord directoryRecord, Audiobook *audiobook) {
    this->record = directoryRecord;
    this->audiobook = audiobook;
}
