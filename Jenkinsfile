pipeline {
  agent { label 'MacMini' }
  options { skipDefaultCheckout() }
  parameters {
    gitParameter(name: 'GIT_TAG',
                 type: 'PT_BRANCH_TAG',
                 description: 'The Git tag to checkout. If not specified "master" will be checkout.',
                 defaultValue: 'master')
    booleanParam(name: 'PUBLISH_RESULTS',
                 description: 'Publish the resulting wheels to Eclipse download and to pypi.org (if not a branch)',
                 defaultValue: false)
    booleanParam(name: 'PYTHON_36',
                 description: 'Build wheel for Python 3.6.',
                 defaultValue: true)
    booleanParam(name: 'PYTHON_37',
                 description: 'Build wheel for Python 3.7.',
                 defaultValue: true)
    booleanParam(name: 'PYTHON_38',
                 description: 'Build wheel for Python 3.8.',
                 defaultValue: true)
    booleanParam(name: 'PYTHON_39',
                 description: 'Build wheel for Python 3.9.',
                 defaultValue: true)
  }
  environment {
      LABEL = get_label()
      MATURIN_PYTHONS_OPT = maturin_python_opt()
  }

  stages {
    stage('Checkout Git TAG') {
      steps {
        deleteDir()
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.GIT_TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/eclipse-zenoh/zenoh-python.git']]
                ])
      }
    }

    stage('MacOS wheels') {
      steps {
        sh '''
        . ~/.zshenv
        export PATH=$PATH:~/miniconda3/envs/zenoh-cp36/bin
        export PATH=$PATH:~/miniconda3/envs/zenoh-cp37/bin
        export PATH=$PATH:~/miniconda3/envs/zenoh-cp38/bin
        export PATH=$PATH:~/miniconda3/envs/zenoh-cp39/bin
        maturin build --release $MATURIN_PYTHONS_OPT
        '''
      }
    }

    stage('Manylinux2010-x64 wheels') {
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/manylinux2010-x64-rust-nightly maturin build --release --manylinux 2010 $MATURIN_PYTHONS_OPT
        '''
      }
    }

    stage('Manylinux2010-i686 wheels') {
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/manylinux2010-i686-rust-nightly maturin build --release --manylinux 2010 $MATURIN_PYTHONS_OPT
        '''
      }
    }

    stage('Manylinux2014-aarch64 wheels') {
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/manylinux2014-aarch64-rust-nightly maturin build --release --manylinux 2014 $MATURIN_PYTHONS_OPT
        '''
      }
    }

    stage('Deploy to download.eclipse.org') {
      when { expression { return params.PUBLISH_RESULTS }}
      steps {
        sshagent ( ['projects-storage.eclipse.org-bot-ssh']) {
          sh '''
            ssh genie.zenoh@projects-storage.eclipse.org rm -fr /home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${LABEL}
            ssh genie.zenoh@projects-storage.eclipse.org mkdir -p /home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${LABEL}
            scp target/wheels/*.whl target/wheels/*.tar.gz genie.zenoh@projects-storage.eclipse.org:/home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${LABEL}/
          '''
        }
      }
    }

    stage('Deploy on pypi.org') {
      when { expression { return params.PUBLISH_RESULTS && !env.GIT_TAG.startsWith('origin/') }}
      steps {
        sh '''
          python3 -m twine upload --repository eclipse-zenoh target/wheels/*.whl target/wheels/*.tar.gz
        '''
      }
    }
  }

  post {
    success {
        archiveArtifacts artifacts: 'target/wheels/*.whl, target/wheels/*.tar.gz', fingerprint: true
    }
  }
}

def get_label() {
    return env.GIT_TAG.startsWith('origin/') ? env.GIT_TAG.minus('origin/') : env.GIT_TAG
}

def maturin_python_opt() {
  String result = '';
  if (env.PYTHON_36) {
    result.concat('-i python3.6 ');
  }
  if (env.PYTHON_37) {
    result.concat('-i python3.7 ');
  }
  if (env.PYTHON_38) {
    result.concat('-i python3.8 ');
  }
  if (env.PYTHON_39) {
    result.concat('-i python3.9 ');
  }
}