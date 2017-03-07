//
// Created by mistlight on 3/5/2017.
//

#ifndef NODOKA_ABSTRACTELEMENTSCANNER_H
#define NODOKA_ABSTRACTELEMENTSCANNER_H

#include <QDir>
#include <QFile>
#include <QString>

class AbstractElementScanner {

public:

    virtual void accept(QDir) = 0;
    virtual void accept(QFile) = 0;
    virtual void accept(QString directory) = 0;

};


#endif //NODOKA_ABSTRACTELEMENTSCANNER_H
