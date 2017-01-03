//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOK_H
#define NODOKANATIVE_AUDIOBOOK_H

#include <QSqlTableModel>
#include <memory>
#include "AudiobookRecord.h"

class Audiobook : public QSqlTableModel {

public:
    Audiobook(QObject *parent = 0);
    void registerAudiobook(std::shared_ptr<QDir> directory);
};


#endif //NODOKANATIVE_AUDIOBOOK_H
