//
// Created by mistlight on 1/29/2017.
//

#include <src/core/AudiobookScan.h>
#include <src/core/directory-element-scan/AddRemoveVisitor.h>
#include "ScanDirectoryTask.h"

void Core::ScanDirectoryTask::run() {
    QString path = this->record.value("full_path").toString();
    AddRemoveVisitor scanner(this->audiobook, path);

    // start scanning from the base directory path
    scanner.accept(path);
}

Core::ScanDirectoryTask::ScanDirectoryTask(QSqlRecord directoryRecord, Audiobook *audiobook) {
    this->record = directoryRecord;
    this->audiobook = audiobook;
}
