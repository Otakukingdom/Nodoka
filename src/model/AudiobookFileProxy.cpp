//
// Created by mistlight on 1/7/17.
//

#include "AudiobookFileProxy.h"

AudiobookFileProxy::AudiobookFileProxy(QSqlRecord record) {
    this->record = record;
    this->isNull = false;
}

QString AudiobookFileProxy::path() {
    return this->record.value("full_path").toString();
}

bool AudiobookFileProxy::getNullState() {
    return this->isNull;
}

AudiobookFileProxy::AudiobookFileProxy() {
    this->record = QSqlRecord();
    this->isNull = true;
}

QString AudiobookFileProxy::name() {
    return this->record.value("name").toString();
}
