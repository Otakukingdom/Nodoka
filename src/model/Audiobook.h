//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOK_H
#define NODOKANATIVE_AUDIOBOOK_H

#include <QSqlTableModel>
#include <memory>
#include "AudiobookRecord.h"
#include "AudiobookFile.h"

class Audiobook : public QSqlTableModel {
    AudiobookFile* audiobookFile;

public:
    Audiobook(AudiobookFile* audiobookFileModel, QObject *parent = 0);
    int getRowForPath(QString path);
    
    void registerAudiobook(QSqlRecord baseDirectoryRecord, std::shared_ptr<QDir> directory);
    void removeAudiobook(QSqlRecord record);
    void removeAudiobook(QString path);
    void removeAudiobook(int row);
};


#endif //NODOKANATIVE_AUDIOBOOK_H
