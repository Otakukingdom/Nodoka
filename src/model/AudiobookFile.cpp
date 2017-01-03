//
// Created by mistlight on 1/2/2017.
//

#include "AudiobookFile.h"


AudiobookFile::AudiobookFile(QObject *parent) : QSqlTableModel(parent) {

}

void AudiobookFile::addAudiobookFile(QString path) {

}

void AudiobookFile::registerAudioBook(int audiobookId, std::shared_ptr<QDir> directory) {

}

int AudiobookFile::getRowForPath(QString path) {
    return 0;
}
