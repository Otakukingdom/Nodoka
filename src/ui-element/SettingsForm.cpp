//
// Created by mistlight on 12/31/2016.
//

#include <QtWidgets/QFileDialog>
#include <QDebug>
#include "SettingsForm.h"

SettingsForm::SettingsForm(Directory* directoryModel, QWidget *parent) :
  QWidget(parent), ui(new Ui::SettingsForm()) {
    ui->setupUi(this);
    ui->listView->setModel(directoryModel);
    this->directoryModel = directoryModel;
    this->setup();
}

SettingsForm::~SettingsForm() {
    delete ui;
}

void SettingsForm::setup() {
    connect(this->ui->addDirectoryButton, &QPushButton::clicked, this, &SettingsForm::performAddDirectory);
}

void SettingsForm::performAddDirectory() {
    qDebug() << "Perform add dir called";
    auto dir = QFileDialog::getExistingDirectory(this, "Select Folder", "", QFileDialog::ShowDirsOnly);
    qDebug() << "Directory is " << dir;
    this->directoryModel->addDirectory(dir);
}
