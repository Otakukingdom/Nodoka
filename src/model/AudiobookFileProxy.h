//
// Created by mistlight on 1/7/17.
//

#ifndef NODOKANATIVE_AUDIOBOOKRECORDPROXY_H
#define NODOKANATIVE_AUDIOBOOKRECORDPROXY_H


#include <QSqlRecord>
#include <QVariant>

class AudiobookFileProxy {
    QSqlRecord record;

public:
    AudiobookFileProxy(QSqlRecord record);
    QString path();

};


#endif //NODOKANATIVE_AUDIOBOOKRECORDPROXY_H
