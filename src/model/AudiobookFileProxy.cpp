//
// Created by mistlight on 1/7/17.
//

#include "AudiobookFileProxy.h"

AudiobookFileProxy::AudiobookFileProxy(QSqlRecord record) {
    this->record = record;
}

QString AudiobookFileProxy::path() {
    return this->record.value("full_path").toString();
}
